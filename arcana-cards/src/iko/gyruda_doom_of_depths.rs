//! Gyruda, Doom of Depths — `{4}{U/B}{U/B}` 6/6 legendary Demon Kraken.
//! Companion — even mana values. When Gyruda enters, each player mills four
//! cards. Put a creature card with an even mana value from among the milled
//! cards onto the battlefield under your control.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gyruda, Doom of Depths");
    let demon = reg.interner_mut().intern("Demon");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(kraken);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U/B}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: "Companion — even mana values" deck-construction keyword is not
    // expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_each_player_mills,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_each_player_mills(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put a creature card with an even mana value from among the
    // milled cards onto the battlefield" — no primitive reanimates a card
    // constrained to those just milled and to an even-mv filter. Each
    // player milling four is expressible.
    let mills: Vec<Effect> = script::all_players(state)
        .into_iter()
        .map(|p| Effect::Mill { player: p, count: 4 })
        .collect();
    vec![Effect::Sequence(mills)]
}
