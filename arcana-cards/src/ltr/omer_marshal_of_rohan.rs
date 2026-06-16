//! Éomer, Marshal of Rohan — `{2}{R}{R}` 4/4 Legendary Human Knight.
//! Haste.
//! Whenever one or more other attacking legendary creatures you control
//! die, untap all creatures you control. After this phase, there is an
//! additional combat phase. This ability triggers only once each turn.
//!
//! The trigger is modeled as a legendary-creature-you-control death
//! (ZoneChange battlefield→graveyard, frequency once-per-turn). The
//! "other" / "attacking" refinements are not expressible on the filter —
//! noted below. The effect untaps all your creatures and grants an
//! additional combat phase.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::objects::NULL_OBJECT_ID;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Éomer, Marshal of Rohan");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP (trigger refinement): "other" (exclude self) and
            // "attacking" cannot be expressed on the death filter; modeled
            // as any legendary creature you control dying.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: untap_and_extra_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn untap_and_extra_combat(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Untap { target: NULL_OBJECT_ID }),
        },
        Effect::AdditionalCombatPhase,
    ]
}
