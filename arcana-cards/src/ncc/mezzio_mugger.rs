//! Mezzio Mugger — `{4}{R}` 3/3 Lizard Rogue.
//!
//! Oracle:
//! * Whenever this creature attacks, exile the top card of each player's
//!   library; you may play those cards this turn (any-color mana). (The
//!   closest primitive, `ImpulseExile`, only exiles from YOUR library; the
//!   each-player exile isn't expressible, so the effect body is GAP'd while
//!   the attack trigger is still wired.)
//! * Blitz {2}{R}. (Blitz is not a usable `KeywordAbility`; `keywords: vec![]`,
//!   GAP'd.)

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Mezzio Mugger");
    let lizard = reg.interner_mut().intern("Lizard");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_exile(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile the top card of EACH player's library, you may play them" —
    // ImpulseExile only covers your own library; the each-player exile-with-
    // play-permission isn't expressible.
    Vec::new()
}
