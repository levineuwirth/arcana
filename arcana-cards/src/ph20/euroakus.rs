//! Euroakus — `{4}{G}{G}` 6/6 Legendary Treefolk Wizard.
//! "When Euroakus enters, create a number of 1/1 blue Human Wizard creature
//!  tokens equal to the number of differently named lands you control.
//!  {4}{G}{U}: Draw a card for each Wizard you control. They each get +1/+1
//!  until end of turn for each card in your hand."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Euroakus");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let wizard = reg.interner_mut().intern("Wizard");
    // pre-intern token subtypes
    let _human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_wizards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{G}{U}: Draw a card for each Wizard you control. They each get +1/+1 until end of turn for each card in your hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{G}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_and_pump_wizards,
            }),
    )
}

fn etb_make_wizards(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: token count = "number of differently named lands you control" —
    // distinct-name counting is not expressible with the available script
    // helpers, so the whole token-creation effect is omitted.
    Vec::new()
}

fn draw_and_pump_wizards(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wizard_filter =
        script::subtype_filter(reg, "Wizard").controlled_by(ControllerConstraint::You);
    let n_wizards = script::count_matching(state, &wizard_filter, ctx.controller);
    // Snapshot hand size BEFORE the draw (documented timing fidelity gap: the
    // oracle pumps using the post-draw hand size).
    let hand = script::hand_size(state, ctx.controller);
    let ids = script::ids_matching(state, &wizard_filter, ctx.controller);

    let mut effects = vec![Effect::DrawCards {
        player: ctx.controller,
        count: n_wizards,
    }];
    if hand > 0 {
        effects.push(Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Pump {
                target: NULL_OBJECT_ID,
                power: hand as i32,
                toughness: hand as i32,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
        });
    }
    effects
}
