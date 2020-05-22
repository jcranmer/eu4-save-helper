use paradox::FixedPoint;
paradox::condition_list!{
    condition(Country, absolutism, FixedPoint);
    condition(Country, exists, String);
    condition(Country, has_country_flag, String);

    condition(Province, continent, String);
}
