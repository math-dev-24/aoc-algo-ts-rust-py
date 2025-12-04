import 'package:http/http.dart' as http;
import 'package:dotenv/dotenv.dart';

class Input {
    final http.Client client;
    final String cookie;
    final String baseUrl;

    Input._({
        required this.client,
        required this.cookie,
        required this.baseUrl,
    });


    factory Input() {
        final env = DotEnv()..load();

        final session = env['SESSION'];

        if (session == null || session.isEmpty) {
            throw Exception("SESSION est vide !");
        }

        final baseUrl = env['URL'];
        if (baseUrl == null || baseUrl.isEmpty) {
            throw Exception("URL est vide !");
        }

        return Input._(
            client: http.Client(),
            cookie: 'session=$session',
            baseUrl: baseUrl
        );
    }

    Future<String> getInput(int year, int day) async {
    final url = '$baseUrl/$year/day/$day/input';
    
    final response = await client.get(
        Uri.parse(url),
        headers: {
            'Cookie': cookie,
        },
    );

    if (response.statusCode != 200) {
        throw Exception('HTTP Error: ${response.statusCode}');
    }

    return response.body;
    }


    void close() {
        client.close();
    }

}