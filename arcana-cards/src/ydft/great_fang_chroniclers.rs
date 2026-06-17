//! Great Fang Chroniclers — `{1}{G}` 2/2 Ape Druid.
//! "Double team" (not a usable KeywordAbility — GAP'd).
//! "{3}{G}: Conjure a card named Muraganda Petroglyphs onto the battlefield,
//! then this creature loses this ability. Activate only as a sorcery."
//! (Conjure is an Arena-only mechanic with no Effect variant — GAP'd.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Great Fang Chroniclers");
    let ape = reg.interner_mut().intern("Ape");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: keyword "Double team" is not a usable KeywordAbility variant.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}: Conjure a card named Muraganda Petroglyphs onto the battlefield, then this creature loses this ability. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: conjure_petroglyphs,
            }),
    )
}

fn conjure_petroglyphs(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name
    // lookup in Effect::execute). The "loses this ability" rider is likewise
    // not expressible.
    Vec::new()
}
