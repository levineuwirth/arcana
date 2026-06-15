//! Wurmweaver Coil — `{4}{G}{G}` enchantment — Aura.
//! "Enchant green creature. Enchanted creature gets +6/+6.
//! {G}{G}{G}, Sacrifice this Aura: Create a 6/6 green Wurm creature
//! token."
//!
//! The +6/+6 is the standard `attached_pt` install. The activated ability
//! pays {G}{G}{G} plus sacrificing this Aura and creates a 6/6 green Wurm
//! token. "Wurm" is interned in `register()` and looked up at resolution.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wurmweaver Coil");
    let aura = reg.interner_mut().intern("Aura");
    let _wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // NOTE: "green creature" color wording approximated by caster's choice
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}{G}{G}, Sacrifice this Aura: Create a 6/6 green Wurm creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{G}{G}").expect("valid cost"),
                    sacrifice: true,
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_wurm,
            }),
    )
}

fn etb_install_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            6,
            6,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn make_wurm(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wurm = reg
        .interner()
        .lookup("Wurm")
        .expect("Wurm interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let token = TokenDefinition {
        name: wurm,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
