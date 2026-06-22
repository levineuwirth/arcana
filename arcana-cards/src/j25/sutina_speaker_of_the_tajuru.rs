//! Sutina, Speaker of the Tajuru — `{2}{G}` 2/2 Legendary Creature — Elf Scout.
//!
//! * When Sutina enters, search your library for a basic land card, put it
//!   onto the battlefield tapped, then shuffle.
//! * Whenever Sutina attacks, you may return a land you control to its
//!   owner's hand. When you do, put a +1/+1 counter on target creature.
//!
//! The ETB tutor is expressed with [`Effect::TutorToBattlefield`] over a
//! basic-land filter (tapped). The attack trigger's "you may return a land
//! you control" is modeled with [`Effect::ChooseAnyNumberFromZone`]
//! (PickAction::ReturnToHand over your lands) followed by a +1/+1 counter on
//! the targeted creature, wrapped in one `Sequence`. The reflexive "When you
//! do" linkage and the strict one-land cap are documented fidelity gaps.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sutina, Speaker of the Tajuru");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_basic,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_bounce_land_then_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

/// ETB: search for a basic land card, put onto battlefield tapped, shuffle.
fn etb_tutor_basic(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
        tapped: true,
    }]
}

/// On attack: you may return a land you control to its owner's hand; if you
/// do, put a +1/+1 counter on target creature.
fn on_attack_bounce_land_then_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "you may return a land you control" — min-0 / max-all pick; the strict
    // one-land cap and the reflexive "When you do" linkage are fidelity gaps.
    vec![Effect::Sequence(vec![
        Effect::ChooseAnyNumberFromZone {
            chooser: trig.controller,
            zone: Zone::Battlefield,
            filter: ObjectFilter::permanent()
                .with_types(TypeLine::LAND.into())
                .controlled_by(ControllerConstraint::You),
            action: PickAction::ReturnToHand,
        },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ])]
}
