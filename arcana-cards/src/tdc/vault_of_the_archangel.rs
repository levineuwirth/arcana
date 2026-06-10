//! Vault of the Archangel — nonbasic land (Dark Ascension).
//! "{T}: Add {C}." and "{2}{W}{B}, {T}: Creatures you control gain
//! deathtouch and lifelink until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vault of the Archangel");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}{B}, {T}: Creatures you control gain deathtouch and lifelink until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: team_deathtouch_lifelink,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn team_deathtouch_lifelink(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::GrantKeyword {
                target: arcana_core::objects::NULL_OBJECT_ID,
                keyword: KeywordAbility::Deathtouch,
                duration: Duration::EndOfTurn,
            }),
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::GrantKeyword {
                target: arcana_core::objects::NULL_OBJECT_ID,
                keyword: KeywordAbility::Lifelink,
                duration: Duration::EndOfTurn,
            }),
        },
    ]
}
