import Image from 'next/image'

interface CardProps {
	img: string
	alt: string
	aspectRatio?: string
	heading: string
	description: string
	category: string
	date?: string
	categoryColor?: 'green' | 'blue' | 'purple' | 'red' | 'yellow' | 'indigo' | 'pink'
	cardBorderRadius?: string
	backgroundColor?: string
}

const NewsCard = ({
	img,
	alt,
	heading,
	description,
	category,
	categoryColor = 'green',
	cardBorderRadius = 'xl',
}: CardProps) => {
	// Create a mapping for the color classes
	const colorClasses = {
		green: {
			text: 'text-green-300',
			bg: 'bg-green-700',
		},
		blue: {
			text: 'text-blue-300',
			bg: 'bg-blue-700',
		},
		purple: {
			text: 'text-purple-300',
			bg: 'bg-purple-700',
		},
		red: {
			text: 'text-red-300',
			bg: 'bg-red-700',
		},
		yellow: {
			text: 'text-yellow-300',
			bg: 'bg-yellow-700',
		},
		indigo: {
			text: 'text-indigo-300',
			bg: 'bg-indigo-700',
		},
		pink: {
			text: 'text-pink-300',
			bg: 'bg-pink-700',
		},
	}

	// Get the appropriate color classes based on categoryColor
	const textColorClass = colorClasses[categoryColor]?.text || 'text-green-300'
	const bgColorClass = colorClasses[categoryColor]?.bg || 'bg-green-700'

	// Handle border radius
	const borderRadiusClass = `rounded-${cardBorderRadius}`

	return (
		<div
			className={`${borderRadiusClass} backdrop-blur-sm bg-white/5 overflow-hidden h-full shadow-lg hover:shadow-purple-500/20 transition-all duration-300 border border-white/10 p-4`}
		>
			<div className="relative h-52 overflow-hidden">
				<Image
					className="object-cover hover:scale-105 transition-transform duration-500 rounded-lg"
					src={img}
					alt={alt}
					fill
					sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
				/>
			</div>
			<div className="p-5 space-y-4 opacity-80">
				<div className="text-white">
					<h2 className="font-bold text-xl leading-tight">{heading}</h2>
				</div>
				<div className="text-gray-300">
					<p className="line-clamp-3 text-sm">{description}</p>
				</div>
				<div className={textColorClass}>
					<span className={`inline-block px-2 py-1 ${bgColorClass} rounded-lg text-xs font-medium`}>
						{category}
					</span>
				</div>
			</div>
		</div>
	)
}

export default NewsCard
