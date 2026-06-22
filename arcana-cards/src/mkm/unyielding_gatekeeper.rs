//! Unyielding Gatekeeper — `{1}{W}` 3/2 Elephant Cleric (white).
//!
//! Oracle:
//! * "Disguise {1}{W}" — face-down/disguise casting is not in the usable
//!   keyword surface (no `KeywordAbility::Disguise`). GAP'd.
//! * "When this creature is turned face up, exile another target nonland
//!   permanent. If you controlled it, return it to the battlefield tapped.
//!   Otherwise, its controller creates a 2/2 white and blue Detective creature
//!   token." — "turned face up" is approximated with `SelfTransforms`. The exile
//!   is emitted faithfully; the conditional "if you controlled it, return it
//!   tapped, otherwise its controller makes a Detective token" rider depends on
//!   prior control of the exiled object, which is not expressible — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unyielding Gatekeeper");
    let elephant = reg.interner_mut().intern("Elephant");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(cleric);

    // GAP: keyword "Disguise {1}{W}" — face-down disguise casting not modeled.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "turned face up" approximated with SelfTransforms.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: None },
                intervening_if: None,
                effect: exile_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
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
    // GAP: "If you controlled it, return it to the battlefield tapped.
    // Otherwise, its controller creates a 2/2 white and blue Detective token."
    // — the conditional rider depends on prior control of the now-exiled object
    // and is not expressible. Only the exile is emitted.
    vec![Effect::ExilePermanent { target: *id }]
}
