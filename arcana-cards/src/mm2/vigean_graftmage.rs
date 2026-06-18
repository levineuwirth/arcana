//! Vigean Graftmage — `{2}{U}` 0/0 Vedalken Wizard Mutant with Graft 2.
//! `{1}{U}: Untap target creature with a +1/+1 counter on it.`
//! (The "+1/+1 counter on it" target restriction is a partial — modeled as
//! target creature.)

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Vigean Graftmage");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Graft(2)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{U}: Untap target creature with a +1/+1 counter on it.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            // Partial: the "with a +1/+1 counter on it" restriction is not in
            // the documented ObjectFilter refinement surface; modeled as any
            // target creature.
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: untap_target,
        }),
    )
}

fn untap_target(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Untap { target: *id }]
}
