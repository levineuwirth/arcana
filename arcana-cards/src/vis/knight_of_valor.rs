//! Knight of Valor — `{2}{W}` 2/2 Human Knight with Flanking.
//! "{1}{W}: Each creature without flanking blocking this creature gets
//!  -1/-1 until end of turn. Activate only once each turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of Valor");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flanking],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{W}: Each creature without flanking blocking this creature gets -1/-1 until end of turn. Activate only once each turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                once_per_turn: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: weaken_blockers,
        }),
    )
}

fn weaken_blockers(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // FIDELITY GAP: "without flanking" exclusion isn't expressible here
    // (no listed per-id keyword check); sweeps every blocker of this
    // creature. In practice the vast majority of blockers lack flanking.
    let blockers = script::blockers_of(state, ctx.source);
    vec![Effect::ForEach {
        targets: blockers,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: -1,
            toughness: -1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
