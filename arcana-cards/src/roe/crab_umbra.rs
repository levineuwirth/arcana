//! Crab Umbra — `{U}` enchantment — Aura.
//! "Enchant creature. {2}{U}: Untap enchanted creature. Umbra armor (If
//!  enchanted creature would be destroyed, instead remove all damage from it
//!  and destroy this Aura.)"
//!
//! The granted host activated ability ("{2}{U}: untap enchanted creature") is
//! an ETB-installed `attached_activated` that untaps the HOST (`ctx.source`).
//! Umbra armor (totem-armor destroy replacement) has no expressible
//! replacement primitive here and is GAP'd.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crab Umbra");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
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
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Umbra armor (totem-armor destroy replacement) — no replacement
    // primitive expressible here.
    let ability = ActivatedAbilityDef {
        text: "{2}{U}: Untap enchanted creature.".into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: untap_host,
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_activated(
            trig.source,
            ability,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn untap_host(
    _state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
