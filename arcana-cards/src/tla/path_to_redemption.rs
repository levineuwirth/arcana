//! Path to Redemption — `{1}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature can't attack or block.
//!  {5}, Sacrifice this Aura: Exile enchanted creature. Create a 1/1
//!  white Ally creature token. Activate only during your turn."
//!
//! Restriction Aura. ETB installs `attached_cant_attack` +
//! `attached_cant_block`. The Aura gains a self-sacrifice activated
//! ability ({5}, sacrifice the Aura) that exiles the enchanted creature
//! (reached via the Aura's `attached_to` before it leaves) and creates a
//! 1/1 white Ally token. "Activate only during your turn" timing is not
//! separately expressible — noted, ability emitted at sorcery speed.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Path to Redemption");
    let aura = reg.interner_mut().intern("Aura");
    let _ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "activate only during your turn" timing not separately
        // expressible.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_pacify,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}, Sacrifice this Aura: Exile enchanted creature. \
                       Create a 1/1 white Ally creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    sacrifice: true,
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_and_make_ally,
            }),
    )
}

fn etb_install_pacify(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_cant_attack(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_cant_block(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn exile_and_make_ally(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ally = reg.interner().lookup("Ally")
        .expect("Ally interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ally);
    let token = TokenDefinition {
        name: ally,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = Vec::new();
    if let Some(host) = state.object_or_lki(ctx.source).and_then(|o| o.attached_to) {
        effects.push(Effect::ExilePermanent { target: host });
    }
    effects.push(Effect::CreateToken {
        controller: ctx.controller,
        token,
    });
    effects
}
