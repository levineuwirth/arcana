//! Spire Phantasm — `{2}{U}{U}` 3/2 blue Gargoyle Illusion with Flying.
//! Draft-matters reveal line (not a gameplay mechanic — omitted).
//! "When this creature enters, if you guessed correctly for a card named Spire
//! Phantasm, draw a card."

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
    let name = reg.interner_mut().intern("Spire Phantasm");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Reveal this card as you draft it. ..." — draft mechanics are not
    // modeled by the engine.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: intervening-if "if you guessed correctly during the draft" —
            // the draft-guess state is not modeled, so the condition can't be
            // checked. Left None and the draw is GAP'd in the body rather than
            // firing unconditionally.
            intervening_if: None,
            effect: maybe_draw,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn maybe_draw(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the draw is gated on a draft-time guess that the engine doesn't
    // track; drawing unconditionally would be wrong, so the effect is omitted.
    Vec::new()
}
