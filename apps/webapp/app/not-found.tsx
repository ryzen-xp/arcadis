'use client'
import { PageWrapper } from '@/components/PageWrapper'
import Link from 'next/link'

export default function NotFound() {
  return (
    <PageWrapper>
      <div className="container mx-auto px-4 py-20 flex flex-col items-center justify-center min-h-screen gap-12">
        <h1 className="text-white font-semibold text-4xl mb-4">404 - Page Not Found</h1>
        <p className="text-white/70 text-lg mb-8">Sorry, the page you are looking for does not exist.</p>
        <Link 
          href="/" 
          className="bg-gradient-to-r from-[#3B82F6] to-[#0E7490] text-white px-8 py-2 rounded-full text-[13.67px] leading-[20px] font-medium transition-all duration-300 hover:opacity-90"
        >
          Back to Home
        </Link>
      </div>
    </PageWrapper>
  )
} 