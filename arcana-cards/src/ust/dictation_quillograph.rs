//! Dictation Quillograph — Artifact — Contraption (Unstable, 2017).
//! "Whenever you crank this Contraption, until end of turn, target
//! creature gains 'Whenever this creature deals combat damage to a
//! player, you may draw a card. If you do, discard a card.'" Contraption
//! cranking is not modeled — per the trigger catalog's closest-variant
//! rule the trigger is wired on `SelfBecomesTapped` with a GAP note (a
//! Contraption with no tap ability is effectively inert). The granted
//! ability is a `GrantTriggeredAbility` for the saboteur loot; the "you
//! may … if you do" choice is modeled as a deterministic draw-then-
//! discard (fidelity note).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dictation Quillograph");
    let contraption = reg.interner_mut().intern("Contraption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "Whenever you crank this Contraption"
                // (Un-set Contraption cranking is not modeled; closest
                // variant used as a placeholder).
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: grant_loot_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            },
        ),
    )
}

fn grant_loot_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantTriggeredAbility {
        target: *id,
        ability: Box::new(TriggeredAbilityDef {
            id: GRANTED_TRIGGER_ID_BASE + 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: granted_loot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        duration: Duration::EndOfTurn,
    }]
}

fn granted_loot(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity note: "you may draw a card. If you do, discard a card" is
    // modeled as a deterministic draw-then-discard.
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
