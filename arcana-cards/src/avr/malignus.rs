//! Malignus — `{3}{R}{R}` */* red Elemental Spirit.
//!
//! Oracle:
//! * "Malignus's power and toughness are each equal to half the highest life
//!   total among your opponents, rounded up." — a characteristic-defining
//!   ability wired at Layer 7a via a SelfEntersBattlefield `self_pt_cda`: the
//!   compute takes the maximum life total among the controller's opponents and
//!   halves it rounding up (`(max + 1) / 2`), setting base P/T to `(n, n)`
//!   (symmetric scalar). Reads life-total state scalars (recursion-proof).
//! * "Damage that would be dealt by this creature can't be prevented." — GAP:
//!   there is no unpreventable-damage continuous-effect primitive available.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malignus");
    let elemental = reg.interner_mut().intern("Elemental");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA — P/T each = ceil(highest opponent life / 2) — resolved at 7a.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_cda,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power and toughness each = half the highest life total among the
// controller's opponents, rounded up.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let highest = script::opponents(s, who)
        .into_iter()
        .map(|p| script::life(s, p))
        .max()
        .unwrap_or(0)
        .max(0);
    let n = (highest + 1) / 2;
    (n, n)
}
