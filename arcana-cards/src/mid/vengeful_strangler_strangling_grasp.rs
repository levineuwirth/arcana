//! Vengeful Strangler // Strangling Grasp
//!
//! Front: {1}{B} Creature — Human Rogue 2/1
//! This creature can't block.
//! When this creature dies, return it to the battlefield transformed under your control
//! attached to target creature or planeswalker an opponent controls.
//!
//! Back: Enchantment — Aura
//! Enchant creature or planeswalker an opponent controls.
//! At the beginning of your upkeep, enchanted permanent's controller sacrifices a nonland
//! permanent of their choice, then that player loses 1 life.
//!
//! Back-face upkeep trigger: wired face-gated to the back face; "enchanted permanent's
//! controller sacrifices a nonland permanent of their choice" is `Effect::ChooseNFromZone`
//! with chooser = the enchanted permanent's controller (read via `attached_to`), action
//! Sacrifice, plus that player loses 1 life. No-op while the Aura is unattached.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vengeful Strangler");
    let sub_human = reg.interner_mut().intern("Human");
    let sub_rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_human);
    subtypes.0.insert(sub_rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "This creature can't block" — static cant-block not modeled as a keyword
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Strangling Grasp");
    let sub_aura = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sub_aura);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    // Front: when this creature dies, return it transformed attached to a target creature
    // or planeswalker an opponent controls.
    // The ZoneChange trigger fires when it moves to the graveyard.
    // Effect::Transform + Attach represent the return-and-attach semantics.
    // GAP: "return it to the battlefield transformed attached to target creature or planeswalker"
    // — ReturnFromGraveyardToBattlefield + Transform + Attach is approximate; targeting
    // constraint (opponent's creature or planeswalker) not enforced in trigger.

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: "At the beginning of your upkeep, enchanted
            // permanent's controller sacrifices a nonland permanent of
            // their choice, then that player loses 1 life."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_strangle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1),
    )
}

fn on_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return it to the battlefield transformed attached to target creature or
    // planeswalker an opponent controls" — full semantics require targeting at trigger
    // resolution; emitting Transform only as approximate skeleton.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}

/// Back face: "…enchanted permanent's controller sacrifices a nonland
/// permanent of their choice, then that player loses 1 life." The chooser
/// is the controller of whatever this Aura is attached to; the filter's
/// controller constraint is evaluated from the CHOOSER's perspective.
fn upkeep_strangle(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = state
        .objects
        .get(trig.source)
        .and_then(|o| o.attached_to)
        .and_then(|host| state.objects.get(host))
        .map(|host| host.controller)
    else {
        // Unattached (e.g. the dies-trigger's attach step is GAP'd) — no
        // enchanted permanent, so no controller to act.
        return Vec::new();
    };
    vec![
        Effect::ChooseNFromZone {
            chooser: p,
            zone: Zone::Battlefield,
            filter: ObjectFilter::permanent()
                .without_types(TypeLine::LAND.into())
                .controlled_by(ControllerConstraint::You),
            min: 1,
            max: 1,
            action: PickAction::Sacrifice,
        },
        Effect::LoseLife { player: p, amount: 1 },
    ]
}
