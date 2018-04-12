import example.gen.github.v3 as v3

if __name__ == "__main__":
    client = v3.Github_Requests()
    rate_limit = client.get_rate_limit()
    print(rate_limit.resources.core)

    for g in client.get_gists_for_user("udoprog"):
        print(g.url)
