scope_list!{
    scope(*, Country, emperor);
    scope(*, Country, revolution_target);
    scope(*, Country, crusade_target);
    scope(Country, Country, all_countries_including_self);
    scope(Country, Country, colonial_parent);
    scope(Country, Province, home_trade_node);
    scope(Province, Country, controller);
    scope(Province, Province, sea_zone);
    scope(Province, Province, area);
    scope(Province, Province, capital_scope);
    scope(Province, Country, most_province_trade_power);
    scope(Province, Country, strongest_trade_power);

    scope_many(Country, Province, active_trade_node);
    scope_many(Country, Country, ally);
    scope_many(Country, Country, coalition_member);
    scope_many(Country, Province, core_province);
    scope_many(Country, Country, country);
    scope_many(Country, Country, elector);
    scope_many(Country, Country, enemy_country);
    scope_many(Country, Province, heretic_province);
    scope_many(Country, Country, known_country);
    scope_many(Country, Country, local_enemy);
    scope_many(Country, Country, neighbor_country);
    scope_many(Country, Province, owned_province);
    scope_many(Country, Province, province);
    scope_many(Country, Country, rival_country);
    scope_many(Country, Province, state_province);
    scope_many(Country, Province, states);
    scope_many(Country, Country, subject_country);
    scope_many(Country, Province, trade_node);
}
