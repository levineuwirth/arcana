//! Furnace Scamp — `{R}` 1/1 red creature. "Whenever this creature deals
//! combat damage to a player, you may sacrifice it. If you do, this creature
//! deals 3 damage to that player."
//!
//! Wired via Effect::OptionalPayment { Sacrifice(Creature) → deal 3 to the
//! damaged player }. Caveat: SacrificeFilter::Creature can't pin the cost to the
//! source ("it") — the chooser is offered any creature they control to sacrifice.

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Furnace Scamp");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_to_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_to_player(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else { return Vec::new(); };
    // "you may sacrifice it. If you do, this creature deals 3 damage to that player."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Creature),
        then: Box::new(Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 3,
            source: trig.source,
        }),
        else_effect: None,
    }]
}
