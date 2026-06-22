//! Aerial Extortionist — `{3}{W}{W}` 4/3 white Bird Soldier.
//!
//! * Flying (keyword).
//! * "Whenever this creature enters or deals combat damage to a player,
//!   exile up to one target nonland permanent. For as long as that card
//!   remains exiled, its owner may cast it." — split into two triggers
//!   (ETB + combat-damage-to-player). The exile is modeled with
//!   `ExilePermanent`; the "its owner may cast it" permission has no
//!   primitive and is GAP'd.
//! * "Whenever another player casts a spell from anywhere other than
//!   their hand, draw a card." — the "from anywhere other than their
//!   hand" zone restriction is not expressible; firing on every
//!   opponent spell would materially over-fire, so the effect is GAP'd
//!   (the opponent-spell-cast trigger is recorded with no effect).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aerial Extortionist");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    let nonland = || TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
        ),
        count: TargetCount::UpTo(1),
        controller: None,
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: exile_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![nonland()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: exile_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![nonland()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: cast_from_elsewhere_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn exile_nonland(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "For as long as that card remains exiled, its owner may cast
    // it." — no exile-with-owner-cast-permission primitive; plain exile.
    vec![Effect::ExilePermanent { target: *id }]
}

fn cast_from_elsewhere_draw(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "from anywhere other than their hand" cast-zone restriction is
    // not expressible; firing the draw on every opponent spell would
    // materially over-fire, so the draw is omitted.
    Vec::new()
}
