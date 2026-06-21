//! The Majestic Duo — `{2}{G}{U}` 2/2 Legendary Human Wizard.
//! "When this enters, if you don't control another permanent named The
//! Majestic Duo, create a token that's a copy of it, except ..."
//! "Whenever this creature deals combat damage to a player, you and
//! that player may each put a permanent card from your hand onto the
//! battlefield."

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Majestic Duo");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Source filter matching this creature by name for the combat-damage
    // trigger (the closest faithful expression of "this creature deals").
    let self_name_filter = ObjectFilter { name: Some(name), ..ObjectFilter::default() };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(if_no_other_majestic_duo),
                effect: copy_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_name_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: both_put_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_no_other_majestic_duo(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let nm = reg.interner().lookup("The Majestic Duo");
    let filter = ObjectFilter { name: nm, ..ObjectFilter::default() }
        .controlled_by(ControllerConstraint::You);
    // True (allow trigger) only if you control AT MOST one — i.e. no
    // OTHER one beyond this entering copy.
    conditions::you_control_at_most(s, you, &filter, 1)
}

fn copy_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "except it's not legendary, it has '<triggered ability>', and
    // it loses all other abilities" — CopyPermanent mints a faithful copy
    // but cannot apply per-copy modifications (drop legendary, swap the
    // ability set). The plain copy is emitted.
    vec![Effect::CopyPermanent { target: trig.source }]
}

fn both_put_permanent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::permanent(),
        tapped: false,
    }];
    if let Some(p) = trig.damaged_player() {
        effects.push(Effect::PutFromHandOntoBattlefield {
            player: p,
            filter: ObjectFilter::permanent(),
            tapped: false,
        });
    }
    effects
}
