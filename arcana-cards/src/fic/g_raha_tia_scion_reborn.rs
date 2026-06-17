//! G'raha Tia, Scion Reborn — `{W}{U}{B}` 2/3 Legendary Cat Wizard.
//! Lifelink.
//! Throw Wide the Gates — Whenever you cast a noncreature spell, you may pay X
//! life, where X is that spell's mana value. If you do, create a 1/1 colorless
//! Hero creature token and put X +1/+1 counters on it. Do this only once each
//! turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("G'raha Tia, Scion Reborn");
    let cat = reg.interner_mut().intern("Cat");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: throw_wide_the_gates,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::OncePerTurn,
            target_requirements: Vec::new(),
        }),
    )
}

fn throw_wide_the_gates(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay X life, where X is that spell's mana value. If you do,
    // create a 1/1 Hero token and put X +1/+1 counters on it." — the payment is a
    // DYNAMIC life cost (X = the cast spell's mana value), and OptionalPayment only
    // accepts a fixed Life(u32); the cast spell's mana value is also not available
    // via the documented PendingTrigger accessors. The whole gated payoff is
    // therefore inexpressible.
    Vec::new()
}
