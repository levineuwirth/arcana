//! Orator of Ojutai — `{1}{W}` 0/4 Creature — Bird Monk.
//! As an additional cost to cast this spell, you may reveal a Dragon
//! card from your hand. (Additional optional cast cost — GAP'd: not
//! expressible as a triggered/activated ability.)
//! Defender, flying.
//! When this creature enters, if you revealed a Dragon card or
//! controlled a Dragon as you cast this spell, draw a card. (The
//! intervening-if depends on the optional reveal made during casting,
//! which the demonstrated `conditions::` predicates can't observe — the
//! gate is GAP'd; the ETB draw is emitted unconditionally as best
//! effort.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orator of Ojutai");
    let bird = reg.interner_mut().intern("Bird");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: optional additional cast cost "you may reveal a Dragon card"
    // is not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: intervening-if "if you revealed a Dragon card or
            // controlled a Dragon as you cast this spell" depends on a
            // cast-time reveal not observable via conditions::.
            intervening_if: None,
            effect: etb_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
