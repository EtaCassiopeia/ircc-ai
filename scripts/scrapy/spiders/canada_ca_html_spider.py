import scrapy
import os

class CanadaSpider(scrapy.Spider):
    name = 'canada-ca-html'
    allowed_domains = ['www.canada.ca']
    start_urls = ['https://www.canada.ca/en/immigration-refugees-citizenship/services/immigrate-canada.html']

    def parse(self, response):
        yield from self.parse_page_content(response)

    def parse_page_content(self, response):
        # Extracting data from the page
        page_title = response.css('title::text').extract_first()
        page_content = response.css('body').extract_first()

        # Determining file path from URL
        url_path = response.url.replace('https://www.canada.ca', '')
        directory, file_name = os.path.split(url_path)
        directory = os.path.join('output-html', *directory.split('/'))
        os.makedirs(directory, exist_ok=True)
        file_path = os.path.join(directory, file_name)

        # Saving data to a file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(f'<title>{page_title}</title>\n{page_content}\n')
        
        # Finding and following new links within page content
        for href in response.css('a::attr(href)').extract():
            next_url_path = response.urljoin(href)
            # Checking if the link resides under the specified initial path
            if next_url_path.startswith('https://www.canada.ca/en/immigration-refugees-citizenship/services/immigrate-canada'):
                yield scrapy.Request(next_url_path, self.parse_page_content)
