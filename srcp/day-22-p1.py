from srcp.utils.input import get_data


def generate_secret_number(initial_secret, iterations=2000, modulo=16777216):
    secret = initial_secret
    for _ in range(iterations):
        secret = (secret ^ (secret * 64)) % modulo
        secret = (secret ^ (secret // 32)) % modulo
        secret = (secret ^ (secret * 2048)) % modulo
    return secret


buyers_secrets = list(map(int, get_data(2024, 22).splitlines()))
result_sum = sum(generate_secret_number(secret) for secret in buyers_secrets)

print(result_sum)