//! Daring Fiendbonder — `{3}{B}` 5/1 black Human Warlock.
//!
//! Oracle:
//! * Haste.
//! * This creature attacks each combat if able. (ETB-installed
//!   `must_attack` continuous effect, CR 508.1a.)
//! * `{1}{B}, Exile this card from your graveyard: Put an
//!   indestructible counter on target creature. Activate only as a
//!   sorcery.` — a graveyard-activated ability (mana + exile-self
//!   cost) that adds a named "indestructible" counter to a creature.
//!
//! Decomposition: Haste → `keywords`; the attack requirement → an ETB
//! `must_attack` continuous effect; the graveyard ability → one
//! `ActivatedAbilityDef`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daring Fiendbonder");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);
    // Intern the named counter at registration so the resolver's
    // read-only `reg.interner().lookup("indestructible")` succeeds.
    let _indestructible = reg.interner_mut().intern("indestructible");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
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
            text: "{1}{B}, Exile this card from your graveyard: Put an indestructible \
                   counter on target creature. Activate only as a sorcery."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: put_indestructible_counter,
        }),
    )
}

fn put_indestructible_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let Some(kind) = reg.interner().lookup("indestructible").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind,
        count: 1,
    }]
}

/// ETB: "This creature attacks each combat if able." (CR 508.1a)
fn install_must_attack(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
