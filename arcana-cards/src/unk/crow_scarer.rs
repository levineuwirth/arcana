//! Crow Scarer — `{4}` 2/4 Creature — Scarecrow.
//! When this creature enters, each player creates two 1/1 black Bird
//! creature tokens with flying.
//! Birds are Cowards in addition to their other types.
//! Cowards can't attack or block creatures you control.
//!
//! Implemented: the ETB trigger — each player creates two 1/1 black Bird
//! tokens with flying.
//! GAP (static): "Birds are Cowards in addition to their other types" is a
//! continuous board-wide type-adding static (no triggered/activated form).
//! GAP (static): "Cowards can't attack or block creatures you control" is a
//! continuous restriction static.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crow Scarer");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);
    // Pre-intern the Bird token subtype for the resolver lookup.
    let _bird = reg.interner_mut().intern("Bird");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_each_player_two_birds,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_each_player_two_birds(
    state: &GameState,
    _trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(bird) = reg.interner().lookup("Bird") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let token = TokenDefinition {
        name: bird,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    let mut out = Vec::new();
    for p in script::all_players(state) {
        out.push(Effect::CreateToken { controller: p, token: token.clone() });
        out.push(Effect::CreateToken { controller: p, token: token.clone() });
    }
    out
}
