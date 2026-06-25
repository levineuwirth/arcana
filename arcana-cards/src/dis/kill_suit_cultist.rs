//! Kill-Suit Cultist — `{R}` 1/1 Goblin Berserker.
//! "This creature attacks each combat if able." (SELF must-attack, installed on ETB)
//! "{B}, Sacrifice this creature: The next time damage would be dealt to target
//! creature this turn, destroy that creature instead."
//!
//! The activation's cost (mana + sacrifice-self, target creature) is wired, but
//! its effect is a damage-replacement-into-destroy rider with no matching Effect
//! — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kill-Suit Cultist");
    let goblin = reg.interner_mut().intern("Goblin");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_must_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
            text: "{B}, Sacrifice this creature: The next time damage would be dealt to target creature this turn, destroy that creature instead.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: damage_to_destroy,
        }),
    )
}

fn install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn damage_to_destroy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "the next time damage would be dealt to target creature this turn,
    // destroy that creature instead" — damage-replacement-into-destroy rider not
    // expressible.
    Vec::new()
}
