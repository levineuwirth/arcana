//! Skarrg Goliath — `{6}{G}{G}` 9/9 Creature — Beast.
//! Trample.
//! Bloodrush — {5}{G}{G}, Discard this card: Target attacking creature
//!   gets +9/+9 and gains trample until end of turn.
//!
//! GAP: Bloodrush is not an expressible KeywordAbility — the keyword
//! line carries only Trample.
//! GAP (fidelity): the activated ability targets "target attacking
//! creature"; the "attacking" restriction is approximated as any target
//! creature (no attacking-only target filter in this surface).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skarrg Goliath");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Bloodrush — {5}{G}{G}, Discard this card: Target attacking creature gets +9/+9 and gains trample until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}{G}{G}").expect("valid cost"),
                discard_self: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Hand,
            is_instant_speed: false,
            face_gate: None,
            effect: bloodrush_pump,
        }),
    )
}

fn bloodrush_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 9,
        toughness: 9,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    }]
}
