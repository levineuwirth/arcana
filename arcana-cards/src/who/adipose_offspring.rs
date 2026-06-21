//! Adipose Offspring — `{3}{W}` 2/2 Alien.
//! Emerge {5}{W} — not in the usable keyword surface → GAP'd.
//! "When this creature enters, create a 2/2 white Alien creature token.
//! If this creature's emerge cost was paid, instead create X of those
//! tokens, where X is the sacrificed creature's toughness."

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Adipose Offspring");
    let alien = reg.interner_mut().intern("Alien");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Emerge {5}{W} — alternative-cast mechanic not in the usable
        // keyword surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_alien,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_alien(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Base case: create one 2/2 white Alien token.
    // GAP: "If this creature's emerge cost was paid, instead create X of
    // those tokens, where X is the sacrificed creature's toughness" — no
    // accessor for whether emerge was paid nor the sacrificed creature's
    // toughness, so only the unconditional single token is emitted.
    let alien = reg.interner().lookup("Alien").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: alien,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
