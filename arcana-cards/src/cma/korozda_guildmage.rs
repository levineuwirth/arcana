//! Korozda Guildmage — `{B}{G}` 2/2 Creature — Elf Shaman.
//! "{1}{B}{G}: Target creature gets +1/+1 and gains intimidate until end of
//! turn." "{2}{B}{G}, Sacrifice a nontoken creature: Create X 1/1 green
//! Saproling creature tokens, where X is the sacrificed creature's toughness."
//!
//! Ability 1 is a targeted pump + Intimidate grant. Ability 2's cost (mana +
//! sacrifice a chosen nontoken creature) is expressible, but its effect needs
//! X = the SACRIFICED creature's toughness, which is gone by resolution and has
//! no ActivationContext accessor — so the token-creation effect is GAP'd (a
//! literal count where the text is dynamic would be materially wrong).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Korozda Guildmage");
    let elf = reg.interner_mut().intern("Elf");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{G}: Target creature gets +1/+1 and gains intimidate until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_intimidate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}{G}, Sacrifice a nontoken creature: Create X 1/1 green Saproling creature tokens, where X is the sacrificed creature's toughness.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}{G}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter::creature().nontoken()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_saprolings,
            }),
    )
}

fn pump_intimidate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Intimidate],
    }]
}

fn make_saprolings(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = the sacrificed creature's toughness. The sacrifice is paid as a
    // cost, so the creature is gone by resolution and there is no
    // ActivationContext accessor for it — the dynamic token count is
    // uncomputable here, so the whole token-creation effect is GAP'd.
    Vec::new()
}
