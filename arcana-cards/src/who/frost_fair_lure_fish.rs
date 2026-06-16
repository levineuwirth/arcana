//! Frost Fair Lure Fish — `{5}{U}{R}` 7/7 Fish.
//! "When this creature enters, create two 1/1 blue Fish creature tokens
//! and create two tapped Treasure tokens." "Fish you control have haste
//! and can't be blocked by Humans." Foretell {3}{U}{R}.
//!
//! ETB wired: two 1/1 blue Fish tokens + two Treasure tokens (the
//! "tapped" rider on the Treasures is a fidelity GAP — CreateCommodityToken
//! mints them untapped). The Fish anthem ("haste and can't be blocked by
//! Humans") is a pure continuous static — GAP'd. Foretell is not in the
//! usable keyword surface — GAP'd.

use arcana_core::effects::{CommodityToken, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frost Fair Lure Fish");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        // GAP: Foretell {3}{U}{R} — not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: static "Fish you control have haste and can't be blocked by
    // Humans" — a pure continuous anthem, not a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let fish = reg.interner().lookup("Fish").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    let fish_token = || Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: fish,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: subtypes.clone(),
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    };
    vec![
        fish_token(),
        fish_token(),
        // GAP: "tapped" Treasure rider — CreateCommodityToken mints them untapped.
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Treasure,
            count: 2,
        },
    ]
}
