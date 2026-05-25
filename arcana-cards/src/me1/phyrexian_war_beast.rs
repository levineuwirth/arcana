//! Phyrexian War Beast — `{3}` colorless 3/4 Artifact Creature — Phyrexian Beast.
//! "When this creature leaves the battlefield, sacrifice a land and this creature
//! deals 1 damage to you."
//!
//! GAP: no TriggerCondition::SelfLeavesBattlefield; using SelfDies as best-effort
//! (covers only dying, not other zone changes). The Sacrifice effect uses a land filter.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian War Beast");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no TriggerCondition::SelfLeavesBattlefield; SelfDies only covers dying
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_leaves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_leaves(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Sacrifice {
            player: trig.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            count: 1,
        },
        Effect::DealDamage {
            target: DamageTarget::Player(trig.controller),
            amount: 1,
            source: trig.source,
        },
    ]
}
