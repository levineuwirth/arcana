//! Genju of the Spires — `{R}` enchantment — Aura.
//! "Enchant Mountain. {2}: Enchanted Mountain becomes a 6/1 red Spirit
//!  creature until end of turn. It's still a land. When enchanted Mountain
//!  is put into a graveyard, you may return this card from your graveyard
//!  to your hand."
//!
//! Land Aura. The ETB installs a host-granted activated ability
//! (`attached_activated`) whose {2} cost runs against the host land
//! (`ctx.source`) and whose effect animates that land into a 6/1 red Spirit
//! creature until end of turn (SetBasePT 6/1, AddType CREATURE — it stays a
//! land, SetColor red, and an EndOfTurn Spirit-subtype continuous effect).
//!
//! GAP: the "When enchanted Mountain is put into a graveyard, you may return
//! this card from your graveyard to your hand" trigger returns the Aura
//! itself from a graveyard — a self-recursion payoff with no expressible
//! attached/host-trigger builder.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Genju of the Spires");
    let aura = reg.interner_mut().intern("Aura");
    // Pre-intern Spirit for the animation's subtype add at activation time.
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "Enchant Mountain" widened to any land (no named-subtype enchant filter).
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
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "when enchanted Mountain is put into a graveyard, return this
    // card from your graveyard to your hand" trigger (Aura self-recursion).
    let ability = ActivatedAbilityDef {
        text: "{2}: Enchanted Mountain becomes a 6/1 red Spirit creature until end of turn. It's still a land.".into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{2}").expect("valid cost"),
            ..Default::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: animate_host,
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_activated(
            trig.source,
            ability,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn animate_host(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let host = ctx.source;
    let mut effects = vec![
        Effect::SetBasePT {
            target: host,
            power: 6,
            toughness: 1,
            duration: Duration::EndOfTurn,
        },
        Effect::AddType {
            target: host,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetColor {
            target: host,
            colors: ColorSet::red(),
            duration: Duration::EndOfTurn,
        },
    ];
    if let Some(spirit) = reg.interner().lookup("Spirit") {
        let mut spirit_set = SubtypeSet::default();
        spirit_set.0.insert(spirit);
        effects.push(Effect::InstallContinuousEffect {
            effect: ContinuousEffect::add_subtypes(
                host,
                host,
                spirit_set,
                Duration::EndOfTurn,
            ),
        });
    }
    effects
}
