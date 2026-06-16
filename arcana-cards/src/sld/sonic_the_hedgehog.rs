//! Sonic the Hedgehog — `{1}{U}{R}{W}` 2/4 Legendary Hedgehog Warrior
//! with Haste.
//! Gotta Go Fast — Whenever Sonic the Hedgehog attacks, put a +1/+1
//! counter on each creature you control with flash or haste.
//! Whenever a creature you control with flash or haste is dealt
//! damage, create a tapped Treasure token.
//!
//! GAP: a "with flash or haste" creature filter is not expressible with
//! the demonstrated ObjectFilter/TargetFilter surface, so both ability
//! payloads (which scope to such creatures) are GAP'd. The "tapped"
//! rider on the Treasure token is also not expressible
//! (CreateCommodityToken mints untapped).

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
    let name = reg.interner_mut().intern("Sonic the Hedgehog");
    let hedgehog = reg.interner_mut().intern("Hedgehog");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hedgehog);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // NOTE: cannot restrict the damaged creature to "you
                // control with flash or haste"; matches any creature
                // dealt damage. Payload is GAP'd regardless.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::permanent(),
                    target_filter: TargetFilter::Creature,
                    combat_only: false,
                },
                intervening_if: None,
                effect: on_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1/+1 counter on each creature you control with flash or
    // haste" — no flash-or-haste creature filter available.
    Vec::new()
}

fn on_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: scoped to "a creature you control with flash or haste";
    // the trigger can't be filtered to such creatures, and the token
    // must enter tapped (no tapped-Treasure primitive). Emitting none
    // to avoid firing on unrelated damage.
    Vec::new()
}
