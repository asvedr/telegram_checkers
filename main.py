# -*- coding: utf-8 -*-

import traceback
import sys
import telebot
import Image
from genImage import genImage
from player import Board
import time

token = '183798409:AAExrQIlrVU5i22FuOEhelMWu4QFyvLLQac'
debug = True
allowTest = True

inds  = 'abcdefgh'
indsG = 'ABCDEFGH'
def symToPos(s):
	try:
		return (inds.index(s[0]), int(s[1]) - 1)
	except:
		return (indsG.index(s[0]), int(s[1]) - 1)
def posToSym((x,y)):
	return inds[x] + str(y + 1)

class Main:
	def __init__(self):
		self.bot   = telebot.TeleBot(token)
		self.state = None
		self.key   = None
		self.store = {}
		self.live  = True
		try:
			self.loadCache()
			print 'cache loaded'
		except:
			print 'cahce not loaded'

		bot = self.bot

		def send_message(id, text):
			print 'TO %s' % id
			print text
			bot.send_message(id, text)

		def sendErr(label):
			print 'INIT FUN %s' % label
			def foo(f):
				def bar(m):
					print 'CALL FUN %s WITH %s' % (label, m.chat.id)
					stat = self.get(m.chat.id)
					try:
						return f(m)
					except Exception as e:
						if debug:
							traceback.print_exc(file=sys.stdout)
						else:
							bot.send_message(m.chat.id, 'CONGRATULATIONS')
							bot.send_message(m.chat.id, 'YOU CRASH ME')
							bot.send_message(m.chat.id, 'THIS IS YOUR LOG:')
							bot.send_message(m.chat.id, e.__repr__())
							bot.send_message(m.chat.id, 'message: "%s", history: "%s"' % (m, stat))
				return bar
			return foo
	
		@bot.message_handler(commands=['start'])
		@sendErr('start')
		def f(m):
			send_message(m.chat.id, "что бы начать новую партию используй /new")

		@bot.message_handler(commands=['new'])
		@sendErr('new')
		def startCmd(mess):
			send_message(mess.chat.id, 'let\'s start your chess party')
			try:
				diff = int(mess.text.split(' ')[1])
			except:
				diff = 3
			send = lambda d: send_message(mess.chat.id, u'Валидное значение в интервале [1..7]. Значение изменено на ' + d)
			if diff < 1:
				diff = 1
				send('1')
			elif diff > 7:
				diff = 7
				send('7')
			self.initParty(mess.chat.id, diff)

		@bot.message_handler(commands=['av'])
		@sendErr('av')
		def steps(mess):
			try:
				fig = mess.text.split(' ')[1]
			except:
				fig = None
			send_message(mess.chat.id, 'available steps:')
			#send_message(mess.chat.id, str(self.steps(mess.chat.id, fig)))
			h = open(self.steps(mess.chat.id, fig), 'rb')
			bot.send_photo(mess.chat.id, h)
			h.close()

		@bot.message_handler(commands=['show'])
		@sendErr('show')
		def show(mess):
			b = self.getBoard(mess.chat.id)
			#bot.send_message(mess.chat.id, board)	
			fname = '%s.png' % mess.chat.id
			genImage(b.data, fname)
			h = open(fname, 'rb')
			bot.send_message(mess.chat.id, self.get(mess.chat.id))
			bot.send_photo(mess.chat.id, h)
			h.close()

		@bot.message_handler(commands=['stat'])
		@sendErr('stat')
		def stat(mess):
			stat = self.getStat(mess.chat.id)
			send_message(mess.chat.id, stat)

		@bot.message_handler(commands=['save'])
		@sendErr('save')
		def save(mess):
			key = self.get(mess.chat.id)
			send_message(mess.chat.id, key)

		@bot.message_handler(commands=['load'])
		@sendErr('load')
		def load(mess):
			t = mess.text.split(' ')
			b = Board()
			ans = b.load(t[1], mess.chat.id)
			if ans[0]:
				send_message(mess.chat.id, u'загружено')
				self.store[mess.chat.id] = t[1]
				self.saveCache()
			else:
				if ans[1] == 'NOTUGAME':
					send_message(mess.chat.id, u'это сохранение не из твоей игры')
				elif ans[1] == 'VALIDERR':
					send_message(mess.chat.id, u'сообщение не валидно')
				else:
					send_message(mess.chat.id, u'не могу распаковать сообщение')
		@bot.message_handler(commands=['stop'])
		@sendErr('stop')
		def stop(mess):
			try:
				key = mess.text.split(' ')[1]
				assert(key == 'Algiz999')
				bot.send_message(mess.chat.id, 'activated')
				self.live = False
			except:
				bot.send_message(mess.chat.id, u'ты врешь')

		@bot.message_handler(func=lambda message: True, content_types=['text'])
		@sendErr('<def>')
		def answer(mess):
			cmd = mess.text.split(' ')
			id  = mess.chat.id
			if len(cmd) == 1:
#				try:
					fig = mess.text
					send_message(mess.chat.id, u'доступные ходы')
					h = open(self.steps(mess.chat.id, fig), 'rb')
					bot.send_photo(mess.chat.id, h)
					h.close()
#				except:
#					bot.send_message(mess.chat.id, 'bad cell')
			elif len(cmd) == 2 and cmd[0] == 'add' and allowTest:
				(x,y) = symToPos(cmd[1])
				b = self.getBoard(id)
				b.data[y][x] = 'w'
				self.store[id] = b.save(id)
				self.saveCache()
				bot.send_message(mess.chat.id, 'ok')
			elif len(cmd) == 2:
				c1 = symToPos(cmd[0])
				c2 = symToPos(cmd[1])
				b = self.getBoard(id)
				print (c1, b.get(c1), c2, b.get(c2))
				if not (c1 in b.available()):
					bot.send_message(id, u'первая клетка не валидна')
					return
				steps = b.stepsFor(c1[0], c1[1], showDead=True)
				for way in steps:
					if way[0][-1] == c2:
						v = b.get(c1)
						if c2[1] == 0:
							b.put(c2, 'W')
						else:
							b.put(c2, v)
						b.put(c1,' ')
						for cd in way[1]:
							b.put(cd, ' ')
						self.store[id] = b.save(id)
						bot.send_message(id, u'думаю')
						b.aiRequest()
						code = b.save(id)
						self.store[id] = code
						bot.send_message(id, code)
						out = str(id) + '.png'
						genImage(b.data, out)
						h = open(out, 'rb')
						bot.send_photo(id, h)
						h.close()
						self.saveCache()
						return
				bot.send_message(id, u'вторая клетка не валидна')
			else:
				bot.send_message(id, u'неизвестная команда')

			#send_message(id, u'не понимаю команду')

		self.bot.polling()
		while self.live:
			time.sleep(100)

	def initParty(self, id, diff=3):
		b = Board()
		b.init(diff)
		self.store[id] = b.save(id)
		self.saveCache()
		
	def get(self, id):
		if id in self.store:
			pass
		else:
			self.initParty(id)
		return self.store[id]
	
	def getBoard(self, id):
		s = self.get(id)
		b = Board()
		b.load(s,id)
		return b

	# available steps
	def steps(self, id, fig):
		out = str(id) + '.png'
		if fig:
			(x,y) = symToPos(fig)
			b = self.getBoard(id)
			#return reduce(lambda a,b: '%s %s' % (a,b), map(posToSym, b.stepsFor(x,y)))
			lst = b.stepsFor(x,y)
			print lst
			genImage(b.data, out, moves=lst)
		else:
			b = self.getBoard(id)
			genImage(b.data, out, av=b.available())
		return out

	def makeStep(self, id, text):
		pass

	def getStat(self, id):
		d = self.getBoard(id).diff
		return u'ИИ считает на %d ходов вперед' % d
	
	def saveCache(self):
		with open('cache.conf','wt') as h:
			h.write(str(self.store))
	
	def loadCache(self):
		with open('cache.conf','rt') as h:
			self.store = eval(h.read())

Main()
