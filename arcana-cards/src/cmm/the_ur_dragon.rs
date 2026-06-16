//! The Ur-Dragon — `{4}{W}{U}{B}{R}{G}` 10/10 Legendary Dragon Avatar
//! (WUBRG) with Flying.
//!
//! * Flying → base keyword.
//! * "Whenever one or more Dragons you control attack, draw that many
//!   cards, then you may put a permanent card from your hand onto the
//!   battlefield." → a `CreatureAttacks` trigger filtered to Dragons
//!   you control; on fire, the controller draws a card and may put a
//!   permanent card from hand onto the battlefield.
//!   FIDELITY GAP: the printed trigger is a single batched
//!   "one or more Dragons attack" event (draw EXACTLY the attacking-
//!   Dragon count, ONE optional permanent put); this fires once per
//!   attacking Dragon, so each draws one card and gets one put. No
//!   batched-attack trigger / "that many" attacker count primitive is
//!   available in the demonstrated API.
//!
//! GAP: "Eminence — … other Dragon spells you cast cost {1} less to
//! cast." is a static cost-reduction (functioning from the command
//! zone or battlefield); the demonstrated Effect catalog has no
//! cost-reduction primitive, so this static is omitted.

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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Ur-Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(avatar);

    let dragons_you_control =
        script::subtype_filter(reg, "Dragon").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: dragons_you_control,
            },
            intervening_if: None,
            effect: on_dragons_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_dragons_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::PutFromHandOntoBattlefield {
            player: trig.controller,
            filter: ObjectFilter::permanent(),
            tapped: false,
        },
    ]
}
