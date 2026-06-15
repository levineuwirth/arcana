//! So Shiny — `{2}{U}` enchantment — Aura.
//! "Enchant creature. When this Aura enters, if you control a token, tap
//!  enchanted creature, then scry 2. Enchanted creature doesn't untap
//!  during its controller's untap step."
//!
//! The ETB "if you control a token" gate is an intervening-if; on a true
//! gate the effect taps the host (reached via `source.attached_to`) and
//! scries 2. "Doesn't untap during its controller's untap step" has no
//! attached continuous-effect builder — GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("So Shiny");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(if_control_token),
                effect: etb_tap_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_control_token(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(s, you, &ObjectFilter::permanent().tokens_only())
}

fn etb_tap_scry(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "doesn't untap during its controller's untap step" — no attached builder.
    let mut out = Vec::new();
    if let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) {
        out.push(Effect::Tap { target: host });
    }
    out.push(Effect::Scry {
        player: trig.controller,
        count: 2,
    });
    out
}
