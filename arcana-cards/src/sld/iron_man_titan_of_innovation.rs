//! Iron Man, Titan of Innovation — `{3}{U}{R}` 4/4 Legendary Artifact
//! Creature — Human Hero.
//!
//! Oracle:
//! * Flying, haste
//! * Genius Industrialist — Whenever Iron Man attacks, create a Treasure
//!   token, then you may sacrifice a noncreature artifact. If you do,
//!   search your library for an artifact card with mana value equal to
//!   1 plus the sacrificed artifact's mana value, put it onto the
//!   battlefield tapped, then shuffle.
//!
//! Flying and Haste are base keywords. The attack trigger creates a
//! Treasure token (the engine wires its activation). The conditional
//! "sacrifice a noncreature artifact, then tutor an artifact whose mana
//! value depends on the sacrificed one's mana value" is GAP'd — there is
//! no sacrifice-then-tutor effect whose tutor cost is parametrized by
//! the sacrificed object's mana value.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Iron Man, Titan of Innovation");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_make_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack_make_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Create a Treasure token.
    // GAP: "then you may sacrifice a noncreature artifact. If you do,
    //      search your library for an artifact card with mana value
    //      equal to 1 plus the sacrificed artifact's mana value, put it
    //      onto the battlefield tapped, then shuffle." — no sacrifice-
    //      then-tutor effect parametrized by the sacrificed object's
    //      mana value.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
