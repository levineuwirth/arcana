//! Security Blockade — `{2}{W}` enchantment — Aura.
//! "Enchant land. When this Aura enters, create a 2/2 white Knight
//! creature token with vigilance. Enchanted land has \"{T}: Prevent the
//! next 1 damage that would be dealt to you this turn.\""
//!
//! `with_enchant(land)` declares the enchant target; the engine attaches
//! the Aura on resolution. The ETB trigger creates the Knight token AND
//! installs an `attached_activated` ability on the enchanted land
//! (`{T}: prevent the next 1 damage to you this turn`).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Security Blockade");
    let aura = reg.interner_mut().intern("Aura");
    let _knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight").expect("Knight interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(knight);
    let token = TokenDefinition {
        name: knight,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_activated(
                trig.source,
                ActivatedAbilityDef {
                    text: "{T}: Prevent the next 1 damage that would be dealt to you this turn."
                        .into(),
                    cost: ActivationCost {
                        mana_cost: ManaCost::parse("{0}").unwrap(),
                        tap: true,
                        ..Default::default()
                    },
                    target_requirements: Vec::new(),
                    is_mana_ability: false,
                    is_loyalty_ability: false,
                    activation_zone: ActivationZone::Battlefield,
                    is_instant_speed: false,
                    face_gate: None,
                    effect: prevent_one,
                },
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}

fn prevent_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PreventDamage {
        target: DamageTarget::Player(ctx.controller),
        amount: Some(1),
        duration: ReplacementDuration::EndOfTurn,
    }]
}
