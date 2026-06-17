//! Wasitora, Nekoru Queen — `{2}{B}{R}{G}` 5/4 Legendary Cat Dragon
//! with Flying and Trample. "Whenever Wasitora deals combat damage to a
//! player, that player sacrifices a creature of their choice. If the
//! player can't, you create a 3/3 black, red, and green Cat Dragon
//! creature token with flying."
//!
//! The combat-damage trigger forces the damaged player to sacrifice a
//! creature. The "if the player can't, create a token" fallback depends
//! on a can't-sacrifice test that has no expressible condition, so that
//! token branch is GAP'd; the forced sacrifice is wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wasitora, Nekoru Queen");
    let cat = reg.interner_mut().intern("Cat");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: damaged_player_sacrifices,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn damaged_player_sacrifices(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else { return Vec::new(); };
    // GAP: "If the player can't, you create a 3/3 black, red, and green Cat
    //      Dragon token with flying." — no can't-sacrifice test to gate the
    //      fallback token; only the forced sacrifice is emitted.
    vec![Effect::Sacrifice {
        player: p,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
