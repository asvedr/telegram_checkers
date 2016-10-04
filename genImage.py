import Image
import ImageDraw
import ImageFont

cellSize   = 30
smallSize  = 0.2
bigSize    = 0.4
fontSize   = 32
fontShiftX = 3
lineWidth  = 4
lineColor  = (0,0,255)

def drawCell(img, x, y, clr):
	for i in range(x * cellSize, (x + 1) * cellSize):
		for j in range(y * cellSize, (y + 1) * cellSize):
			img[i,j] = clr
whiteCell = lambda img, x, y: drawCell(img, x, y, (210, 210, 210))
blackCell = lambda img, x, y: drawCell(img, x, y, (110, 110, 110))
blueCell  = lambda img, x, y: drawCell(img, x, y, (000, 000, 250))

white = (150, 250, 150)
black = (000, 000, 000)

def drawSmall(img, cx, cy, clr):
	midX = cellSize * (cx + 0.5)
	midY = cellSize * (cy + 0.5)
	sqRad = (cellSize * bigSize) ** 2
	for x in range(cellSize * cx, cellSize * (cx + 1)):
		for y in range(cellSize * cy, cellSize * (cy + 1)):
			sqD = (x - midX) ** 2 + (y - midY) ** 2
			if sqD < sqRad:
				img[x,y] = clr

def drawBig(img, cx, cy, clr):
	drawSmall(img, cx, cy, clr)
	midX = cellSize * (cx + 0.5)
	midY = cellSize * (cy + 0.5)
	sqRad = (cellSize * smallSize) ** 2
	for x in range(cellSize * cx, cellSize * (cx + 1)):
		for y in range(cellSize * cy, cellSize * (cy + 1)):
			sqD = (x - midX) ** 2 + (y - midY) ** 2
			if sqD < sqRad:
				img[x,y] = (255,255,255)

def symDrawer(img):
	draw = ImageDraw.Draw(img)
	font = ImageFont.truetype('UbuntuMono-Bold.ttf', fontSize)
	def func(x, y, t):
		draw.text((x * cellSize + fontShiftX, y * cellSize), t, (0,0,0), font=font)
	return func

def drawLine(img, (x0,y0), (x1,y1)):
	x0 = int((x0 + 1.5) * cellSize)
	y0 = int((y0 + 1.5) * cellSize)
	x1 = int((x1 + 1.5) * cellSize)
	y1 = int((y1 + 1.5) * cellSize)
	def dist(x,y):
		top = (y0 - y1)*x + (x1 - x0)*y + (x0*y1 - x1*y0)
		bot = (x1 - x0) ** 2 + (y1 - y0) ** 2
		return abs(top) / (bot ** 0.5)
	rad = lineWidth / 2
	minX = min(x0,x1) - rad
	minY = min(y0,y1) - rad
	maxX = max(x0,x1) + rad
	maxY = max(y0,y1) + rad
	for x in range(minX,maxX):
		for y in range(minY,maxY):
			if dist(x,y) <= rad:
				img[x,y] = lineColor

def genImage(board, filename, **args):
	img = Image.new('RGB', (cellSize * 10, cellSize * 10), 'white')
	px = img.load()

	available = args['av'] if 'av' in args else []
	if 'moves' in args:
		for way in args['moves']:
			available.append(way[-1])

	for y in range(1,9):
		for x in range(1,9):
			if y % 2 == x % 2:
				whiteCell(px, x, y)
			else:
				if (x - 1, y - 1) in available:
					blueCell(px, x, y)
				else:
					blackCell(px, x, y)
			c = board[y - 1][x - 1]
			if c == 'w':
				drawSmall(px, x, y, white)
			elif c == 'W':
				drawBig(px, x, y, white)
			elif c == 'b':
				drawSmall(px, x, y, black)
			elif c == 'B':
				drawBig(px, x, y, black)
	if 'moves' in args:
		for way in args['moves']:
			prevP = way[0]
			for i in range(1, len(way)):
				newP = way[i]
				drawLine(px, prevP, newP)
				prevP = newP

	drawer = symDrawer(img)

	syms = 'ABCDEFGH'
	for i in range(8):
		drawer(i + 1, 0, syms[i])
		drawer(i + 1, 9, syms[i])
		drawer(0, i + 1, str(i + 1))
		drawer(9, i + 1, str(i + 1))

	img.save(filename)
	return img

def test():
	b = [
			'        ',
			' wWbB   ',
			'        ',
			'        ',
			'        ',
			' wWbB   ',
			'        ',
			'        '
		]
	genImage(b, 'out.png', av=[(2,1)], moves=[[(2,1),(2,2),(3,3)]])

#test()
