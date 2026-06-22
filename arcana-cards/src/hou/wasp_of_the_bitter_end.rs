//! Wasp of the Bitter End — `{1}{B}` 2/1 Insect Horror with Flying.
//! "Whenever you cast a Bolas planeswalker spell, you may sacrifice this
//!  creature. If you do, destroy target creature."
//!
//! Flying is a base keyword. The cast trigger fires on a Bolas
//! planeswalker spell you cast and destroys the target creature. The
//! "you may sacrifice this creature. If you do" optional self-sacrifice
//! cost-gate has no demonstrated primitive (sacrifice is not an
//! `OptionalPaymentKind`), so that gate is GAP'd while the destroy
//! payload — the meaningful effect — is wired.

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
    let name = reg.interner_mut().intern("Wasp of the Bitter End");
    let insect = reg.interner_mut().intern("Insect");
    let horror = reg.interner_mut().intern("Horror");
    let bolas = reg.interner_mut().intern("Bolas");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let bolas_spell = ObjectFilter::new()
        .with_types(TypeLine::PLANESWALKER.into())
        .with_subtype_sym(bolas)
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(bolas_spell),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: destroy_target,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "you may sacrifice this creature. If you do, …" — the
            // optional self-sacrifice gate is not expressible (sacrifice is
            // not an OptionalPaymentKind); the destroy payload below is wired
            // unconditionally on the trigger.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn destroy_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}
