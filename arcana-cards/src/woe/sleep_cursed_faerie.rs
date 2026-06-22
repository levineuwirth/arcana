//! Sleep-Cursed Faerie — `{U}` 3/3 blue Faerie Wizard with Flying and
//! Ward {2}.
//!
//! Oracle:
//! * Flying, ward {2}.
//! * This creature enters tapped with three stun counters on it.
//! * {1}{U}: Untap this creature.
//!
//! Flying and Ward {2} land on the keyword line; the mana-only untap is a
//! plain activated ability on this source.
//!
//! GAP: "enters tapped with three stun counters on it" — the
//! enters-tapped-with-counters face spec is not part of the documented
//! effect/keyword surface, so the ETB tap and stun counters are omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sleep-Cursed Faerie");
    let faerie = reg.interner_mut().intern("Faerie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{U}: Untap this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: untap_self,
        }),
    )
}

fn untap_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Untap { target: ctx.source }]
}
