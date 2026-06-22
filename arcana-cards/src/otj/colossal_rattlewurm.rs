//! Colossal Rattlewurm — `{2}{G}{G}` 6/5 Creature — Wurm.
//!
//! * Colossal Rattlewurm has flash as long as you control a Desert. (GAP:
//!   conditional static flash-granting with a board condition — no
//!   triggered/activated primitive expresses it, omitted.)
//! * Trample.
//! * {1}{G}, Exile this card from your graveyard: Search your library for a
//!   Desert card, put it onto the battlefield tapped, then shuffle.
//!     (Graveyard-activated; exile-self cost; tutors a Desert onto the
//!      battlefield tapped.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Colossal Rattlewurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let _desert = reg.interner_mut().intern("Desert");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}, Exile this card from your graveyard: Search your \
                       library for a Desert card, put it onto the battlefield \
                       tapped, then shuffle."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_desert,
            }),
    )
}

fn tutor_desert(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let desert = match reg.interner().lookup("Desert") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_subtype_sym(desert);
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter,
        tapped: true,
    }]
}
