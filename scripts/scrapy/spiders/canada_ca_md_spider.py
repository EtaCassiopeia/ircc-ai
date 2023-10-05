import scrapy
import os
import html2text

class CanadaSpider(scrapy.Spider):
    name = 'canada-ca-md'
    allowed_domains = ['www.canada.ca']
    start_urls = ['https://www.canada.ca/en/immigration-refugees-citizenship/services/immigrate-canada.html']

    def parse(self, response):
        yield from self.parse_page_content(response)

    def parse_page_content(self, response):
        # Extracting HTML data from the page
        page_html = response.text
        
        # Converting HTML to Markdown
        converter = html2text.HTML2Text()
        converter.ignore_links = False  # You can set this to True if you want to ignore links
        page_markdown = converter.handle(page_html)

        # Determining file path from URL
        url_path = response.url.replace('https://www.canada.ca', '')
        directory, file_name = os.path.split(url_path)
        directory = os.path.join('output-md', *directory.split('/'))
        os.makedirs(directory, exist_ok=True)
        file_path = os.path.join(directory, file_name.replace('.html', '.md'))

        # Saving Markdown data to a file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(page_markdown)
        
        # Finding and following new links within page content
        for href in response.css('a::attr(href)').extract():
            next_url_path = response.urljoin(href)
            # Checking if the link resides under the specified initial path
            if next_url_path.startswith('https://www.canada.ca/en/immigration-refugees-citizenship/services/immigrate-canada'):
                yield scrapy.Request(next_url_path, self.parse_page_content)
