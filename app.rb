
require 'sinatra'
require 'json'

# Configure Sinatra
set :port, 4567
set :bind, '0.0.0.0'
set :public_folder, 'web'

# Serve static files from web directory
get '/' do
  send_file File.join(settings.public_folder, 'index.html')
end

get '/ohm' do
  send_file File.join(settings.public_folder, 'ohm.html')
end

# Health check endpoint
get '/health' do
  content_type :json
  { status: 'ok', message: 'LiquidDoc Parser server running!' }.to_json
end

puts "🚀 Starting LiquidDoc Parser server on http://localhost:#{settings.port}"
puts "📁 Serving files from: #{settings.public_folder}"
