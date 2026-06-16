//! Drana, Kalastria Bloodchief — `{3}{B}{B}` 4/4 Legendary Creature — Vampire Shaman.
//! Flying.
//! "{X}{B}{B}: Target creature gets -0/-X until end of turn and Drana gets
//! +X/+0 until end of turn."

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drana, Kalastria Bloodchief");
    let vampire = reg.interner_mut().intern("Vampire");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}{B}{B}: Target creature gets -0/-X until end of turn and Drana gets +X/+0 until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{X}{B}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: drain_target,
        }),
    )
}

fn drain_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = ctx.x_value.unwrap_or(0);
    vec![Effect::Sequence(vec![
        Effect::Pump {
            target: *id,
            power: 0,
            toughness: -(x as i32),
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::Pump {
            target: ctx.source,
            power: x as i32,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
    ])]
}
