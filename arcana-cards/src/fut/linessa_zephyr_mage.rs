//! Linessa, Zephyr Mage — `{3}{U}` 3/3 Legendary Human Wizard.
//! `{X}{U}{U}, {T}: Return target creature with mana value X to its
//! owner's hand.` (mv=X filter is a GAP — bounce on a target creature.)
//! Grandeur — `Discard another card named Linessa, Zephyr Mage:` then a
//! cascading bounce by the target player; GAP: the per-type repeat
//! sequence chosen by the target player is not expressible.
//! GAP: Grandeur keyword itself is not in the usable keyword surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Linessa, Zephyr Mage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{U}{U}, {T}: Return target creature with mana value X to its owner's hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{U}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bounce_target,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Grandeur — Discard another card named Linessa, Zephyr Mage: Target player returns a creature they control to its owner's hand, then repeats this process for an artifact, an enchantment, and a land.".into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter {
                        name: reg.interner().lookup("Linessa, Zephyr Mage"),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grandeur_bounce,
            }),
    )
}

fn bounce_target(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "mana value X" filter on the target not expressible — bounce any target creature.
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}

fn grandeur_bounce(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: target player chooses a creature/artifact/enchantment/land they
    // control to return in sequence — per-type, player-chosen bounce chain
    // is not expressible.
    Vec::new()
}
