//! Oriss, Samite Guardian — `{1}{W}{W}` Legendary 1/3 Human Cleric.
//! "{T}: Prevent all damage that would be dealt to target creature
//! this turn." and a Grandeur ability ("Discard another card named
//! Oriss…: Target player can't cast spells this turn, and creatures
//! that player controls can't attack this turn.").
//!
//! The prevention ability is wired. Grandeur is GAP'd: the keyword is
//! not in the supported surface, the "discard a card named X" cost has
//! no matching ActivationCost shape, and "can't cast spells / creatures
//! can't attack this turn" has no matching Effect.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oriss, Samite Guardian");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Grandeur keyword not in supported KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: Grandeur activated ability — "discard a card named X" cost
    // not expressible, and "can't cast spells / creatures can't attack"
    // has no Effect.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Prevent all damage that would be dealt to target creature this turn."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_all_damage,
            }),
    )
}

fn prevent_all_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
