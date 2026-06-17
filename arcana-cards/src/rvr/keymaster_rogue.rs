//! Keymaster Rogue — `{3}{U}` 3/2 Human Rogue.
//! This creature can't be blocked.
//! When this creature enters, return a creature you control to its owner's hand.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keymaster Rogue");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "This creature can't be blocked" — static, applied to self.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_unblockable_and_bounce,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_unblockable_and_bounce(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // ETB: return a creature you control to its owner's hand.
    // The static "can't be blocked" is folded in here as a self-targeting,
    // while-on-battlefield CantBeBlocked (engine idiom for a creature's own
    // unblockable static).
    let ids = script::ids_matching(
        state,
        &arcana_core::targets::ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut out = vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }];
    // Return one creature you control (prefer one other than this; else self).
    if let Some(pick) = ids.iter().copied().find(|id| *id != trig.source).or_else(|| ids.first().copied()) {
        out.push(Effect::ReturnToHand { target: pick });
    }
    out
}
