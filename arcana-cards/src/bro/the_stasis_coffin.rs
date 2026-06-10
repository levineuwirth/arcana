//! The Stasis Coffin — `{3}` legendary artifact.
//! "{2}, {T}, Exile The Stasis Coffin: You gain protection from everything
//! until your next turn." Player protection-from-everything is approximated
//! as broad damage prevention to you (the prevention half); the
//! can't-be-targeted half and the until-your-next-turn duration are GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Stasis Coffin");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, {T}, Exile The Stasis Coffin: You gain protection from everything until your next turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                tap: true,
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: protection_from_everything,
        }),
    )
}

fn protection_from_everything(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "protection from everything" — only the prevent-all-damage-to-you
    // half is expressible; can't-be-targeted / can't-be-attached / can't-be-
    // blocked-by are unmodeled, and the duration is "until your next turn"
    // but only ReplacementDuration::EndOfTurn exists.
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::permanent(),
        target_filter: TargetFilter::Player,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
